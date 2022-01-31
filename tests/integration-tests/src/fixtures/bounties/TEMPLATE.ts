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

export interface BountyParams {
  asMember: MemberId
  // TODO
}

export class CreateBountiesFixture extends StandardizedFixture {
  protected bountiesParams: BountyParams[]

  public constructor(api: Api, query: QueryNodeApi, bountiesParams: BountyParams[]) {
    super(api, query)
    this.bountiesParams = bountiesParams
  }

  // Assertions:

  protected assertQueryNodeEventIsValid(qEvent, i: number): void {
    // TODO
  }

  protected assertQueriedBountiesAreValid(qBounties, qEvents): void {
    // TODO
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()

    // Query the events
    // TODO call this.assertQueryNodeEventsAreValid(qEvents)

    // Query the posts
    // TODO
    this.assertQueriedBountiesAreValid(qBounties, qEvents)
  }

  // Execute method dependencies:

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    // TODO
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return await Promise.all(
      this.bountiesParams.map(async ({ asMember }) =>
        (await this.api.query.members.membershipById(asMember)).controller_account.toString()
      )
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveBountyEventDetails(result, /* TODO */)
  }
}
