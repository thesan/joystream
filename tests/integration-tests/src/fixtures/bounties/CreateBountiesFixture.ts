import { SubmittableExtrinsic } from '@polkadot/api/types'
import { ISubmittableResult } from '@polkadot/types/types/'
import { MemberId } from '@joystream/types/common'
import { BountyMetadata } from '@joystream/metadata-protobuf'
import BN from 'bn.js'
import { assert } from 'chai'

import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { BountyCreatedEventFieldsFragment, BountyFieldsFragment } from '../../graphql/generated/queries'
import { BountyCreatedEventDetails } from '../../types'
import { StandardizedFixture } from '../../Fixture'
import { Utils } from '../../utils'
import { BountyStage } from '../../graphql/generated/schema'

type PerpetualFunding = { target: number }
type LimitedFunding = {
  minFundingAmount: number
  maxFundingAmount: number
  fundingPeriod: number
}

export interface BountyParams {
  params: {
    creator: MemberId
    oracle?: MemberId
    contractWhitelist?: MemberId[]
    funding: PerpetualFunding | LimitedFunding
    cherry: BN
    entrantStake: BN
    workPeriod: number
    judgingPeriod: number
  }
  metadata: Partial<BountyMetadata>
}

export class CreateBountiesFixture extends StandardizedFixture {
  protected events: BountyCreatedEventDetails[] = []
  protected bountiesParams: BountyParams[]

  public constructor(api: Api, query: QueryNodeApi, bountiesParams: BountyParams[]) {
    super(api, query)
    this.bountiesParams = bountiesParams
  }

  // Assertions:

  protected assertQueryNodeEventIsValid(qEvent: BountyCreatedEventFieldsFragment, i: number): void {
    console.log('assertQueryNodeEventIsValid 0')
    assert.equal(qEvent.bounty.id, String(this.events[i].bountyId))
    console.log('assertQueryNodeEventIsValid 1')
  }

  protected assertQueriedBountiesAreValid(qBounties: BountyFieldsFragment[]): void {
    this.events.map(({ bountyId }, i) => {
      const qBounty = qBounties.find(({ id }) => id === String(bountyId))
      const { params, metadata } = this.bountiesParams[i]

      console.log('assertQueriedBountiesAreValid 0')
      Utils.assert(qBounty, 'Query node: Bounty not found')
      console.log('assertQueriedBountiesAreValid 1')
      assert.equal(qBounty.creator?.id, params.creator.toString())
      console.log('assertQueriedBountiesAreValid 2')
      assert.equal(qBounty.oracle?.id, params.oracle?.toString())
      console.log('assertQueriedBountiesAreValid 3')
      assert.equal(qBounty.title, metadata?.title)
      console.log('assertQueriedBountiesAreValid 4')
      assert.equal(qBounty.description, metadata?.description)
      console.log('assertQueriedBountiesAreValid 5')
      assert.equal(qBounty.bannerImageUri, metadata?.bannerImageUri)
      console.log('assertQueriedBountiesAreValid 6')
      // assert.equal(qBounty.discussionThreadId, metadata?.discussionThread)
      assert.equal(qBounty.cherry, params.cherry)
      assert.equal(qBounty.totalFunding, params.cherry)
      params.contractWhitelist?.forEach((memberId, i) => {
        assert(qBounty.contractType.whitelist?.[i].id, String(memberId))
      })
      assert.include(qBounty.fundingType, params.funding)
      assert.equal(qBounty.entrantStake, params.entrantStake)
      assert.equal(qBounty.entries, null)
      assert.equal(qBounty.contributions, null)
      assert.equal(qBounty.judgingPeriod, params.judgingPeriod)
      assert.equal(qBounty.workPeriod, params.workPeriod)
      assert.equal(qBounty.stage, BountyStage.Funding)
    })
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()

    // Query the events
    await this.query.tryQueryWithTimeout(
      () => this.query.getBountyCreatedEventByIds(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the posts
    const qBounties = await this.query.getBountiesByIds(this.events.map((e) => e.bountyId))
    this.assertQueriedBountiesAreValid(qBounties)
  }

  // Execute method and dependencies:

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.bountiesParams.map(({ params, metadata }) => {
      const { creator, oracle, funding, contractWhitelist } = params
      const oracleType = oracle ? 'Member' : 'Council'
      const fundingType = 'target' in funding ? 'Perpetual' : 'Limited'
      const contractType = contractWhitelist ? 'Closed' : 'Open'

      return this.api.tx.bounty.createBounty(
        Utils.camelToSnakeCaseProps({
          ...params,
          creator: { Member: creator },
          oracle: { [oracleType]: oracle ?? null },
          fundingType: { [fundingType]: Utils.camelToSnakeCaseProps(funding) },
          contractType: { [contractType]: Utils.whenDefined(contractWhitelist, Utils.asBtreeSet(MemberId)) },
        }),
        Utils.metadataToBytes(BountyMetadata, metadata)
      )
    })
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return await Promise.all(
      this.bountiesParams.map(async ({ params }) =>
        (await this.api.query.members.membershipById(params.creator)).controller_account.toString()
      )
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<BountyCreatedEventDetails> {
    return this.api.retrieveBountyCreatedEventDetails(result)
  }
}
