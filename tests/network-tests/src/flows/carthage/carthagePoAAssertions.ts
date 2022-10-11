import { expect, assert } from 'chai'
import { extendDebug } from '../../Debugger'
import { FixtureRunner } from '../../Fixture'
import { FlowProps } from '../../Flow'
import { u32 } from '@polkadot/types'
import { BN } from 'bn.js'
import { BondingSucceedsFixture } from '../../fixtures/staking/BondingSucceedsFixture'
import { ValidatingSucceedsFixture } from '../../fixtures/staking/ValidatingSucceedsFixture'
import { NominatingSucceedsFixture } from '../../fixtures/staking/NominatingSucceedsFixture'

export default async function carthagePoAAssertions({ api, query, env }: FlowProps): Promise<void> {
  const debug = extendDebug('flow: constant Authorities in PoA')
  debug('started')
  api.enableDebugTxLogs()

  // ----------------------- ARRANGE -----------------------------
  const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))
  const sleepTimeSeconds = 10
  const nominatorBond = await api.query.staking.minNominatorBond()
  const validatorBond = await api.query.staking.minValidatorBond()
  const [nominatorAccount, validatorAccount] = (await api.createKeyPairs(2)).map(({ key }) => key.address)
  const forceEra = await api.getForceEra()
  const pastAuthorities = await api.getBabeAuthorities()
  assert(forceEra.isForceNone)

  // -------------------------- ACT -------------------------------

  const nominatorBondingSucceedsFixture = new BondingSucceedsFixture(api, {
    stash: nominatorAccount,
    controller: nominatorAccount,
    bondAmount: nominatorBond,
  })
  const nominatorFixture = new FixtureRunner(nominatorBondingSucceedsFixture)
  await nominatorFixture.run()

  const validatorBondingSucceedsFixture = new BondingSucceedsFixture(api, {
    stash: validatorAccount,
    controller: validatorAccount,
    bondAmount: validatorBond,
  })
  const validatorFixture = new FixtureRunner(validatorBondingSucceedsFixture)
  await validatorFixture.run()

  const validatorCandidatingSucceedsFixture = new ValidatingSucceedsFixture(
    api,
    api.createType('PalletStakingValidatorPreferences', {
      'commission': 10,
      'blocked': false,
    }),
    validatorAccount
  )
  const candidationFixture = new FixtureRunner(validatorCandidatingSucceedsFixture)
  await candidationFixture.run()

  const nominatorCandidatingSucceedsFixture = new NominatingSucceedsFixture(api, [validatorAccount], nominatorAccount)
  const nominationFixture = new FixtureRunner(nominatorCandidatingSucceedsFixture)
  await nominationFixture.run()

  // wait for era duration blocks
  let currentSessionIndex = await api.getCurrentSessionIndex()
  while (currentSessionIndex.toBn() < new BN(6)) {
    sleep(sleepTimeSeconds * 1000)
    currentSessionIndex = await api.getCurrentSessionIndex()
  }

  // -------------------------- ASSERT ----------------------------

  // 1. Authorities are constant
  // 1.a. babe authorities are constant
  const currentAuthorities = await api.getBabeAuthorities()
  expect(pastAuthorities).to.be.deep.equal(currentAuthorities)

  // 1.b. Queued keys (for next session) and current session keys are the same
  const sessionAuthorities = await api.getSessionAuthorities()
  const queuedKeys = await api.getQueuedKeys()
  expect(queuedKeys).to.be.deep.equal(sessionAuthorities)

  // 2. Next Era starting session index is none
  const activeEra = await api.getActiveEra()
  if (activeEra.isSome) {
    const { index } = activeEra.unwrap()
    const nextEraIndex = index.addn(1)
    const nextEraStartSessionIndex = await api.getErasStartSessionIndex(nextEraIndex as u32)
    assert.equal(index.toNumber(), 0)
    assert(nextEraStartSessionIndex.isNone)
  }
}
