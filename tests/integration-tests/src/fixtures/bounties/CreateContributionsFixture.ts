import { SubmittableExtrinsic } from '@polkadot/api/types'
import { ISubmittableResult } from '@polkadot/types/types/'
import { MemberId } from '@joystream/types/common'
import BN from 'bn.js'
import { assert } from 'chai'

import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails, MetadataInput } from '../../types'
import { StandardizedFixture } from '../../Fixture'
import { Utils } from '../../utils'
import { BountyId } from '@joystream/types/bounty'

export interface ContributionParams {
  bounty: BountyId
  contributor: MemberId
  amount: BN
}

export class CreateContributionsFixture extends StandardizedFixture {
  protected contributionsParams: ContributionParams[]

  public constructor(api: Api, query: QueryNodeApi, contributionsParams: ContributionParams[]) {
    super(api, query)
    this.contributionsParams = contributionsParams
  }

  // Assertions:

  protected assertQueryNodeEventIsValid(qEvent, i: number): void {
    // TODO
  }

  protected assertQueriedCreateContributionsAreValid(qContributions, qEvents): void {
    // TODO
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()

    // Query the events
    // TODO call this.assertQueryNodeEventsAreValid(qEvents)

    // Query the posts
    // TODO
    this.assertQueriedBountiesAreValid(qContributions, qEvents)
  }

  // Execute method dependencies:

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.contributionsParams.map(({ contributor, bounty, amount }) =>
      this.api.tx.bounty.fundBounty({ Member: contributor }, bounty, amount)
    )
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return await Promise.all(
      this.contributionsParams.map(async ({ contributor }) =>
        (await this.api.query.members.membershipById(contributor)).controller_account.toString()
      )
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveBountyEventDetails(result, /* TODO */)
  }
}
