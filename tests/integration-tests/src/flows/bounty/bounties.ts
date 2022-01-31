import Long from 'long'
import { BountyParams, CreateBountiesFixture } from '../../fixtures/bounties/CreateBountiesFixture'
import { InitializeForumFixture } from '../../fixtures/forum'
// import { BuyMembershipHappyCaseFixture } from '../../fixtures/membership'
import { extendDebug } from '../../Debugger'
import { FixtureRunner } from '../../Fixture'
import { FlowProps } from '../../Flow'

export default async function ({ api, query }: FlowProps): Promise<void> {
  const debug = extendDebug(`flow:threads`)
  debug('Started')
  api.enableDebugTxLogs()

  const initializeForumFixture = new InitializeForumFixture(api, query, {
    numberOfForumMembers: 5,
    numberOfCategories: 1,
    threadsPerCategory: 3,
  })
  await new FixtureRunner(initializeForumFixture).runWithQueryNodeChecks()

  const memberIds = initializeForumFixture.getCreatedForumMemberIds()
  const threadIds = initializeForumFixture.getCreatedThreadIds().map(String)

  const { minCherryLimit, minFundingLimit, minWorkEntrantStake } = api.consts.bounty
  const bounties: BountyParams[] = [
    {
      params: {
        creator: memberIds[0],
        oracle: memberIds[1],
        funding: { target: Number(minFundingLimit) },
        cherry: minCherryLimit,
        entrantStake: minWorkEntrantStake,
        workPeriod: 30,
        judgingPeriod: 20,
      },
      metadata: {
        title: 'First bounty',
        description: 'Aliquip nisi magna occaecat sit.',
        bannerImageUri: '',
        discussionThread: Long.fromString(threadIds[0]),
      },
    },
    {
      params: {
        creator: memberIds[1],
        funding: {
          minFundingAmount: Number(minFundingLimit),
          maxFundingAmount: Number(minFundingLimit) * 2,
          fundingPeriod: 4,
        },
        contractWhitelist: memberIds.slice(1),
        cherry: minCherryLimit.muln(2),
        entrantStake: minWorkEntrantStake.muln(1.5),
        workPeriod: 10,
        judgingPeriod: 5,
      },
      metadata: {
        title: 'Second bounty',
        description: 'Dolore ad elit velit do reprehenderit anim pariatur.',
        bannerImageUri: '',
        discussionThread: Long.fromString(threadIds[1]),
      },
    },
    {
      params: {
        creator: memberIds[2],
        oracle: memberIds[3],
        funding: { target: Number(minFundingLimit) * 1.5 },
        cherry: minCherryLimit,
        entrantStake: minWorkEntrantStake,
        workPeriod: 8,
        judgingPeriod: 7,
      },
      metadata: {
        title: 'Third bounty',
        description: 'Ad id ipsum aute aliqua esse minim.',
        bannerImageUri: '',
        discussionThread: Long.fromString(threadIds[2]),
      },
    },
  ]
  const createBountiesFixture = new CreateBountiesFixture(api, query, bounties)
  const createBountiesRunner = new FixtureRunner(createBountiesFixture)
  await createBountiesRunner.runWithQueryNodeChecks()
}
