import { EventContext, StoreContext, DatabaseManager } from '@joystream/hydra-common'
import { CURRENT_NETWORK, deserializeMetadata, genericEventFields } from './common'
import BN from 'bn.js'
import { FindConditions } from 'typeorm'

import {
  // Council events
  AnnouncingPeriodStartedEvent,
  NotEnoughCandidatesEvent,
  VotingPeriodStartedEvent,
  NewCandidateEvent,
  NewCouncilElectedEvent,
  NewCouncilNotElectedEvent,
  CandidacyStakeReleaseEvent,
  CandidacyWithdrawEvent,
  CandidacyNoteSetEvent,
  RewardPaymentEvent,
  BudgetBalanceSetEvent,
  BudgetRefillEvent,
  BudgetRefillPlannedEvent,
  BudgetIncrementUpdatedEvent,
  CouncilorRewardUpdatedEvent,
  RequestFundedEvent,

  // Referendum events
  ReferendumStartedEvent,
  ReferendumStartedForcefullyEvent,
  RevealingStageStartedEvent,
  ReferendumFinishedEvent,
  VoteCastEvent,
  VoteRevealedEvent,
  StakeReleasedEvent,

  // Council & referendum structures
  ReferendumStageRevealingOptionResult,

  // Council & referendum schema types
  CouncilStageUpdate,
  CouncilStageAnnouncing,
  CouncilStage,
  ElectionProblem,
  Candidate,
  CouncilMember,
  ElectionRound,
  ElectedCouncil,
  CouncilStageElection,
  VariantNone,
  CastVote,
  CandidacyNoteMetadata,
  CandidacyStatusActive,
  CandidacyStatusWithdrawn,
  CandidacyStatusElected,
  CandidacyStatusLost,

  // Misc
  Membership,
} from 'query-node/dist/model'
import { Council, Referendum } from './generated/types'
import { CouncilCandidacyNoteMetadata } from '@joystream/metadata-protobuf'
import { isSet } from '@joystream/metadata-protobuf/utils'

/////////////////// Common - Gets //////////////////////////////////////////////

/*
  Retrieves the member record by its id.
*/
async function getMembership(store: DatabaseManager, memberId: string): Promise<Membership | undefined> {
  const member = await store.get(Membership, { where: { id: memberId } })

  if (!member) {
    throw new Error(`Membership not found. memberId '${memberId}'`)
  }

  return member
}

/*
  Retrieves the council candidate by its member id. Returns the last record for the member
  if the election round isn't explicitly set.
*/
async function getCandidate(
  store: DatabaseManager,
  memberId: string,
  electionRound?: ElectionRound
): Promise<Candidate> {
  const where = { memberId: memberId } as FindConditions<Candidate>
  if (electionRound) {
    where.electionRound = electionRound
  }

  const candidate = await store.get(Candidate, { where, order: { createdAt: 'DESC' } })

  if (!candidate) {
    throw new Error(`Candidate not found. memberId '${memberId}' electionRound '${electionRound?.id}'`)
  }

  return candidate
}

/*
  Retrieves the member's last council member record.
*/
async function getCouncilMember(store: DatabaseManager, memberId: string): Promise<CouncilMember> {
  const councilMember = await store.get(CouncilMember, {
    where: { memberId: memberId },
    order: { createdAt: 'DESC' },
  })

  if (!councilMember) {
    throw new Error(`Council member not found. memberId '${memberId}'`)
  }

  return councilMember
}

/*
  Returns the current election round record.
*/
async function getCurrentElectionRound(store: DatabaseManager): Promise<ElectionRound> {
  const electionRound = await store.get(ElectionRound, { order: { cycleId: 'DESC' } })

  if (!electionRound) {
    throw new Error(`No election round found`)
  }

  return electionRound
}

/*
  Returns the last council stage update.
*/
async function getCurrentStageUpdate(store: DatabaseManager): Promise<CouncilStageUpdate> {
  const stageUpdate = await store.get(CouncilStageUpdate, { order: { changedAt: 'DESC' } })

  if (!stageUpdate) {
    throw new Error('No stage update found.')
  }

  return stageUpdate
}

/*
  Returns current elected council record.
*/
async function getCurrentElectedCouncil(
  store: DatabaseManager,
  canFail: boolean = false
): Promise<ElectedCouncil | undefined> {
  const electedCouncil = await store.get(ElectedCouncil, { order: { electedAtBlock: 'DESC' } })

  if (!electedCouncil && !canFail) {
    throw new Error('No council is elected.')
  }

  return electedCouncil
}

/*
  Returns the last vote cast in an election by the given account. Returns the last record for the account
  if the election round isn't explicitly set.
*/
async function getAccountCastVote(
  store: DatabaseManager,
  account: string,
  electionRound?: ElectionRound
): Promise<CastVote> {
  const where = { castBy: account } as FindConditions<Candidate>
  if (electionRound) {
    where.electionRound = electionRound
  }

  const castVote = await store.get(CastVote, { where, order: { createdAt: 'DESC' } })

  if (!castVote) {
    throw new Error(
      `No vote cast by the given account in the curent election round. accountId '${account}', cycleId '${electionRound?.cycleId}'`
    )
  }

  return castVote
}

/*
  Vote power calculation should correspond to implementation of `referendum::Trait<ReferendumInstance>`
  in `runtime/src/lib.rs`.
*/
function calculateVotePower(accountId: string, stake: BN): BN {
  return stake
}

/*
  Custom typeguard for council stage - announcing candidacy.
*/
function isCouncilStageAnnouncing(councilStage: typeof CouncilStage): councilStage is CouncilStageAnnouncing {
  return councilStage.isTypeOf == 'CouncilStageAnnouncing'
}

/////////////////// Common /////////////////////////////////////////////////////

/*
  Creates new council stage update record.
*/
async function updateCouncilStage(
  store: DatabaseManager,
  councilStage: typeof CouncilStage,
  blockNumber: number,
  electionProblem?: ElectionProblem
): Promise<void> {
  const electedCouncil = await getCurrentElectedCouncil(store, true)
  if (!electedCouncil) {
    return
  }

  const councilStageUpdate = new CouncilStageUpdate({
    stage: councilStage,
    changedAt: new BN(blockNumber),
    electionProblem,
    electedCouncil,
  })

  await store.save<CouncilStageUpdate>(councilStageUpdate)
}

/*
  Concludes current election round and starts the next one.
*/
async function startNextElectionRound(
  store: DatabaseManager,
  electedCouncil: ElectedCouncil,
  blockNumber: number,
  electionProblem?: ElectionProblem
): Promise<ElectionRound> {
  // finish last election round
  const lastElectionRound = await getCurrentElectionRound(store)
  lastElectionRound.isFinished = true
  lastElectionRound.nextElectedCouncil = electedCouncil

  // save last election
  await store.save<ElectionRound>(lastElectionRound)

  // create election round record
  const electionRound = new ElectionRound({
    cycleId: lastElectionRound.cycleId + 1,
    isFinished: false,
    castVotes: [],
    electedCouncil,
    candidates: [],
  })

  // save new election
  await store.save<ElectionRound>(electionRound)

  // update council stage
  const stage = new CouncilStageAnnouncing()
  stage.candidatesCount = new BN(0)
  await updateCouncilStage(store, stage, blockNumber, electionProblem)

  return electionRound
}

/*
  Converts successful council candidate records to council member records.
*/
async function convertCandidatesToCouncilMembers(
  store: DatabaseManager,
  candidates: Candidate[],
  blockNumber: number
): Promise<CouncilMember[]> {
  const councilMembers = await candidates.reduce(async (councilMembersPromise, candidate) => {
    const councilMembers = await councilMembersPromise

    // cast to any needed because member is not eagerly loaded into candidate object
    const member = new Membership({ id: (candidate as any).memberId.toString() })

    const councilMember = new CouncilMember({
      // id: candidate.id // TODO: are ids needed?
      stakingAccountId: candidate.stakingAccountId,
      rewardAccountId: candidate.rewardAccountId,
      member,
      stake: candidate.stake,

      lastPaymentBlock: new BN(blockNumber),

      unpaidReward: new BN(0),
      accumulatedReward: new BN(0),
    })

    return [...councilMembers, councilMember]
  }, Promise.resolve([] as CouncilMember[]))

  return councilMembers
}

/////////////////// Council events /////////////////////////////////////////////

/*
  The event is emitted when a new round of elections begins (can be caused by multiple reasons) and candidates can announce
  their candidacies.
*/
export async function council_AnnouncingPeriodStarted({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [] = new Council.AnnouncingPeriodStartedEvent(event).params

  const announcingPeriodStartedEvent = new AnnouncingPeriodStartedEvent({
    ...genericEventFields(event),
  })

  await store.save<AnnouncingPeriodStartedEvent>(announcingPeriodStartedEvent)

  // specific event processing

  // restart elections
  const electedCouncil = (await getCurrentElectedCouncil(store))!
  await startNextElectionRound(store, electedCouncil, event.blockNumber)
}

/*
  The event is emitted when a candidacy announcment period has ended, but not enough members announced.
*/
export async function council_NotEnoughCandidates({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [] = new Council.NotEnoughCandidatesEvent(event).params

  const notEnoughCandidatesEvent = new NotEnoughCandidatesEvent({
    ...genericEventFields(event),
  })

  await store.save<NotEnoughCandidatesEvent>(notEnoughCandidatesEvent)

  // specific event processing

  // restart elections
  const electedCouncil = (await getCurrentElectedCouncil(store))!
  await startNextElectionRound(store, electedCouncil, event.blockNumber, ElectionProblem.NOT_ENOUGH_CANDIDATES)
}

/*
  The event is emitted when a new round of elections begins (can be caused by multiple reasons).
*/
export async function council_VotingPeriodStarted({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [numOfCandidates] = new Council.VotingPeriodStartedEvent(event).params

  const votingPeriodStartedEvent = new VotingPeriodStartedEvent({
    ...genericEventFields(event),
    numOfCandidates,
  })

  await store.save<VotingPeriodStartedEvent>(votingPeriodStartedEvent)

  // specific event processing

  // add stage update record
  const stage = new CouncilStageElection()
  stage.candidatesCount = new BN(numOfCandidates.toString()) // toString() is needed to duplicate BN

  await updateCouncilStage(store, stage, event.blockNumber)
}

/*
  The event is emitted when a member announces candidacy to the council.
*/
export async function council_NewCandidate({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing - init

  const [memberId, stakingAccount, rewardAccount, balance] = new Council.NewCandidateEvent(event).params
  const member = await getMembership(store, memberId.toString())

  // specific event processing

  // increase candidate count in stage update record
  const lastStageUpdate = await getCurrentStageUpdate(store)
  if (!isCouncilStageAnnouncing(lastStageUpdate.stage)) {
    throw new Error(`Unexpected council stage "${lastStageUpdate.stage.isTypeOf}"`)
  }

  lastStageUpdate.stage.candidatesCount = new BN(lastStageUpdate.stage.candidatesCount).add(new BN(1))
  await store.save<CouncilStageUpdate>(lastStageUpdate)

  const electionRound = await getCurrentElectionRound(store)

  // prepare note metadata record (empty until explicitily set via different extrinsic)
  const noteMetadata = new CandidacyNoteMetadata({
    bulletPoints: [],
  })
  await store.save<CandidacyNoteMetadata>(noteMetadata)

  // save candidate record
  const candidate = new Candidate({
    stakingAccountId: stakingAccount.toString(),
    rewardAccountId: rewardAccount.toString(),
    member,
    status: new CandidacyStatusActive(),
    electionRound,
    stake: balance,
    stakeLocked: true,
    votePower: new BN(0),
    noteMetadata,
    votesRecieved: [],
  })
  await store.save<Candidate>(candidate)

  // common event processing - save

  const newCandidateEvent = new NewCandidateEvent({
    ...genericEventFields(event),
    candidate,
    stakingAccount: stakingAccount.toString(),
    rewardAccount: rewardAccount.toString(),
    balance,
  })

  await store.save<NewCandidateEvent>(newCandidateEvent)
}

/*
  The event is emitted when the new council is elected. Sufficient members were elected and there is no other problem.
*/
export async function council_NewCouncilElected({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing - init

  const [memberIds] = new Council.NewCouncilElectedEvent(event).params
  const electedMemberIds = memberIds.map((item) => item.toString())
  const members = await store.getMany(Membership, { where: { id: electedMemberIds } })

  // specific event processing

  // mark old council as resinged
  const oldElectedCouncil = await getCurrentElectedCouncil(store, true)
  if (oldElectedCouncil) {
    oldElectedCouncil.isResigned = true
    oldElectedCouncil.endedAtBlock = event.blockNumber
    oldElectedCouncil.endedAtTime = new Date(event.blockTimestamp)
    oldElectedCouncil.endedAtNetwork = CURRENT_NETWORK
    await store.save<ElectedCouncil>(oldElectedCouncil)
  }

  // get election round and its candidates
  const electionRound = await getCurrentElectionRound(store)

  const where = { electionRoundId: electionRound.id, status_json: { isTypeOf_eq: 'CandidacyStatusActive' } }
  const candidates = await store.getMany(Candidate, { where })

  const electedCandidates = candidates.filter((candidate) => electedMemberIds.includes(candidate.member.id))

  // Set candidates elected status
  electedCandidates.forEach((candidate) => {
    candidate.status = new CandidacyStatusElected()
  })

  // Update candidates status
  await Promise.all(
    candidates.map(async (candidate) => {
      if (!(candidate.status instanceof CandidacyStatusElected)) {
        candidate.status = new CandidacyStatusLost()
      }
      await store.save<Candidate>(candidate)
    })
  )

  // create new council record
  const electedCouncil = new ElectedCouncil({
    councilMembers: await convertCandidatesToCouncilMembers(store, electedCandidates, event.blockNumber),
    updates: [],
    electedAtBlock: event.blockNumber,
    electedAtTime: new Date(event.blockTimestamp),
    electedAtNetwork: CURRENT_NETWORK,
    councilElections: oldElectedCouncil?.nextCouncilElections || [],
    nextCouncilElections: [],
    isResigned: false,
  })

  await store.save<ElectedCouncil>(electedCouncil)

  // save new council members
  await Promise.all(
    (electedCouncil.councilMembers || []).map(async (councilMember) => {
      councilMember.electedInCouncil = electedCouncil

      await store.save<CouncilMember>(councilMember)
    })
  )

  // end the last election round and start new one
  await startNextElectionRound(store, electedCouncil, event.blockNumber)

  // unset `isCouncilMember` sign for old council's members
  const oldElectedMembers = await store.getMany(Membership, { where: { isCouncilMember: true } })
  await Promise.all(
    oldElectedMembers.map(async (member) => {
      member.isCouncilMember = false

      await store.save<Membership>(member)
    })
  )

  // set `isCouncilMember` sign for new council's members
  await Promise.all(
    (electedCouncil.councilMembers || []).map(async (councilMember) => {
      const member = councilMember.member
      member.isCouncilMember = true

      await store.save<Membership>(member)
    })
  )

  // common event processing - save

  const newCouncilElectedEvent = new NewCouncilElectedEvent({
    ...genericEventFields(event),
    electedCouncil,
  })

  await store.save<NewCouncilElectedEvent>(newCouncilElectedEvent)
}

/*
  The event is emitted when the new council couldn't be elected because not enough candidates received some votes.
  This can be vaguely translated as the public not having enough interest in the candidates.
*/
export async function council_NewCouncilNotElected({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [] = new Council.NewCouncilNotElectedEvent(event).params

  const newCouncilNotElectedEvent = new NewCouncilNotElectedEvent({
    ...genericEventFields(event),
  })

  await store.save<NewCouncilNotElectedEvent>(newCouncilNotElectedEvent)

  // specific event processing

  // restart elections
  const electedCouncil = (await getCurrentElectedCouncil(store))!
  await startNextElectionRound(store, electedCouncil, event.blockNumber, ElectionProblem.NEW_COUNCIL_NOT_ELECTED)
}

/*
  The event is emitted when the member is releasing it's candidacy stake that is no longer needed.
*/
export async function council_CandidacyStakeRelease({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [memberId] = new Council.CandidacyStakeReleaseEvent(event).params
  const candidate = await getCandidate(store, memberId.toString()) // get last member's candidacy record

  const candidacyStakeReleaseEvent = new CandidacyStakeReleaseEvent({
    ...genericEventFields(event),
    candidate,
  })

  await store.save<CandidacyStakeReleaseEvent>(candidacyStakeReleaseEvent)

  // specific event processing

  // update candidate info about stake lock
  candidate.stakeLocked = false
  await store.save<Candidate>(candidate)
}

/*
  The event is emitted when the member is revoking its candidacy during a candidacy announcement stage.
*/
export async function council_CandidacyWithdraw({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [memberId] = new Council.CandidacyWithdrawEvent(event).params
  const candidate = await getCandidate(store, memberId.toString())

  const candidacyWithdrawEvent = new CandidacyWithdrawEvent({
    ...genericEventFields(event),
    candidate,
  })

  await store.save<CandidacyWithdrawEvent>(candidacyWithdrawEvent)

  // specific event processing

  // mark candidacy as withdrawn
  const electionRound = await getCurrentElectionRound(store)
  candidate.status = new CandidacyStatusWithdrawn()
  await store.save<Candidate>(candidate)
}

/*
  The event is emitted when the candidate changes its candidacy note.
*/
export async function council_CandidacyNoteSet({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [memberId, note] = new Council.CandidacyNoteSetEvent(event).params

  // load candidate recored
  const electionRound = await getCurrentElectionRound(store)
  const candidate = await getCandidate(store, memberId.toString(), electionRound)

  // unpack note's metadata and save it to db
  const metadata = deserializeMetadata(CouncilCandidacyNoteMetadata, note)
  const noteMetadata = candidate.noteMetadata
  // `XXX || (null as any)` construct clears metadata if requested (see https://github.com/Joystream/hydra/issues/435)
  noteMetadata.header = isSet(metadata?.header) ? metadata?.header || (null as any) : metadata?.header
  noteMetadata.bulletPoints = metadata?.bulletPoints || []
  noteMetadata.bannerImageUri = isSet(metadata?.bannerImageUri)
    ? metadata?.bannerImageUri || (null as any)
    : metadata?.bannerImageUri
  noteMetadata.description = isSet(metadata?.description)
    ? metadata?.description || (null as any)
    : metadata?.description
  await store.save<CandidacyNoteMetadata>(noteMetadata)

  // save metadata set by this event
  const noteMetadataSnapshot = new CandidacyNoteMetadata({
    header: metadata?.header || undefined,
    bulletPoints: metadata?.bulletPoints || [],
    bannerImageUri: metadata?.bannerImageUri || undefined,
    description: metadata?.description || undefined,
  })

  await store.save<CandidacyNoteMetadata>(noteMetadataSnapshot)

  const candidacyNoteSetEvent = new CandidacyNoteSetEvent({
    ...genericEventFields(event),
    candidate,
    noteMetadata: noteMetadataSnapshot,
  })

  await store.save<CandidacyNoteSetEvent>(candidacyNoteSetEvent)

  // no specific event processing
}

/*
  The event is emitted when the council member receives its reward.
*/
export async function council_RewardPayment({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [memberId, rewardAccount, paidBalance, missingBalance] = new Council.RewardPaymentEvent(event).params
  const councilMember = await getCouncilMember(store, memberId.toString())

  const rewardPaymentEvent = new RewardPaymentEvent({
    ...genericEventFields(event),
    councilMember,
    rewardAccount: rewardAccount.toString(),
    paidBalance,
    missingBalance,
  })

  await store.save<RewardPaymentEvent>(rewardPaymentEvent)

  // specific event processing

  // update (un)paid reward info
  councilMember.accumulatedReward = councilMember.accumulatedReward.add(paidBalance)
  councilMember.unpaidReward = missingBalance
  councilMember.lastPaymentBlock = new BN(event.blockNumber)
  await store.save<CouncilMember>(councilMember)
}

/*
  The event is emitted when a new budget balance is set.
*/
export async function council_BudgetBalanceSet({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [balance] = new Council.BudgetBalanceSetEvent(event).params

  const budgetBalanceSetEvent = new BudgetBalanceSetEvent({
    ...genericEventFields(event),
    balance,
  })

  await store.save<BudgetBalanceSetEvent>(budgetBalanceSetEvent)

  // no specific event processing
}

/*
  The event is emitted when a planned budget refill occurs.
*/
export async function council_BudgetRefill({ event, store }: EventContext & StoreContext): Promise<void> {
  const [balance] = new Council.BudgetRefillEvent(event).params

  const budgetRefillEvent = new BudgetRefillEvent({
    ...genericEventFields(event),
    balance,
  })

  await store.save<BudgetRefillEvent>(budgetRefillEvent)

  // no specific event processing
}

/*
  The event is emitted when a new budget refill is planned.
*/
export async function council_BudgetRefillPlanned({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [nextRefillInBlock] = new Council.BudgetRefillPlannedEvent(event).params

  const budgetRefillPlannedEvent = new BudgetRefillPlannedEvent({
    ...genericEventFields(event),
    nextRefillInBlock: nextRefillInBlock.toNumber(),
  })

  await store.save<BudgetRefillPlannedEvent>(budgetRefillPlannedEvent)

  // no specific event processing
}

/*
  The event is emitted when a regular budget increment amount is updated.
*/
export async function council_BudgetIncrementUpdated({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [amount] = new Council.BudgetIncrementUpdatedEvent(event).params

  const budgetIncrementUpdatedEvent = new BudgetIncrementUpdatedEvent({
    ...genericEventFields(event),
    amount,
  })

  await store.save<BudgetIncrementUpdatedEvent>(budgetIncrementUpdatedEvent)

  // no specific event processing
}

/*
  The event is emitted when the reward amount for council members is updated.
*/
export async function council_CouncilorRewardUpdated({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [rewardAmount] = new Council.CouncilorRewardUpdatedEvent(event).params

  const councilorRewardUpdatedEvent = new CouncilorRewardUpdatedEvent({
    ...genericEventFields(event),
    rewardAmount,
  })

  await store.save<CouncilorRewardUpdatedEvent>(councilorRewardUpdatedEvent)

  // no specific event processing
}

/*
  The event is emitted when funds are transfered from the council budget to an account.
*/
export async function council_RequestFunded({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [account, amount] = new Council.RequestFundedEvent(event).params

  const requestFundedEvent = new RequestFundedEvent({
    ...genericEventFields(event),
    account: account.toString(),
    amount,
  })

  await store.save<RequestFundedEvent>(requestFundedEvent)

  // no specific event processing
}

/////////////////// Referendum events //////////////////////////////////////////

/*
  The event is emitted when the voting stage of elections starts.
*/
export async function referendum_ReferendumStarted({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [winningTargetCount] = new Referendum.ReferendumStartedEvent(event).params

  const referendumStartedEvent = new ReferendumStartedEvent({
    ...genericEventFields(event),
    winningTargetCount,
  })

  await store.save<ReferendumStartedEvent>(referendumStartedEvent)

  // no specific event processing
}

/*
  The event is emitted when the voting stage of elections starts (in a fail-safe way).
*/
export async function referendum_ReferendumStartedForcefully({
  event,
  store,
}: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [winningTargetCount] = new Referendum.ReferendumStartedForcefullyEvent(event).params

  const referendumStartedForcefullyEvent = new ReferendumStartedForcefullyEvent({
    ...genericEventFields(event),
    winningTargetCount,
  })

  await store.save<ReferendumStartedForcefullyEvent>(referendumStartedForcefullyEvent)

  // no specific event processing
}

/*
  The event is emitted when the vote revealing stage of elections starts.
*/
export async function referendum_RevealingStageStarted({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [] = new Referendum.RevealingStageStartedEvent(event).params

  const revealingStageStartedEvent = new RevealingStageStartedEvent({
    ...genericEventFields(event),
  })

  await store.save<RevealingStageStartedEvent>(revealingStageStartedEvent)

  // no specific event processing
}

/*
  The event is emitted when referendum finished and all revealed votes were counted.
*/
export async function referendum_ReferendumFinished({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [optionResultsRaw] = new Referendum.ReferendumFinishedEvent(event).params

  const members = await store.getMany(Membership, {
    where: { id: optionResultsRaw.map((item) => item.option_id.toString()) },
  })

  const referendumFinishedEvent = new ReferendumFinishedEvent({
    ...genericEventFields(event),
    optionResults: optionResultsRaw.map(
      (item, index) =>
        new ReferendumStageRevealingOptionResult({
          votePower: item.vote_power,
          option: members.find((tmpMember) => tmpMember.id.toString() == optionResultsRaw[index].option_id.toString()),
        })
    ),
  })

  await store.save<ReferendumFinishedEvent>(referendumFinishedEvent)

  // no specific event processing
}

/*
  The event is emitted when a vote is casted in the council election.
*/
export async function referendum_VoteCast({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing - init

  const [account, hash, stake] = new Referendum.VoteCastEvent(event).params
  const votePower = calculateVotePower(account.toString(), stake)

  // specific event processing

  const electionRound = await getCurrentElectionRound(store)

  const castVote = new CastVote({
    commitment: hash.toString(),
    electionRound,
    stake,
    stakeLocked: true,
    castBy: account.toString(),
    votePower: votePower,
  })
  await store.save<CastVote>(castVote)

  // common event processing - save

  const voteCastEvent = new VoteCastEvent({
    ...genericEventFields(event),
    castVote,
  })

  await store.save<VoteCastEvent>(voteCastEvent)
}

/*
  The event is emitted when a previously casted vote is revealed.
*/
export async function referendum_VoteRevealed({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing - init

  const [account, memberId, salt] = new Referendum.VoteRevealedEvent(event).params
  const member = await getMembership(store, memberId.toString())

  // specific event processing

  // read vote info
  const electionRound = await getCurrentElectionRound(store)
  const candidate = await getCandidate(store, memberId.toString(), electionRound)
  const castVote = await getAccountCastVote(store, account.toString(), electionRound)

  // update cast vote's voteFor info
  castVote.voteFor = candidate
  await store.save<CastVote>(castVote)

  candidate.votePower = candidate.votePower.add(castVote.votePower)
  await store.save<Candidate>(candidate)

  // common event processing - save

  const voteRevealedEvent = new VoteRevealedEvent({
    ...genericEventFields(event),
    castVote,
  })

  await store.save<VoteRevealedEvent>(voteRevealedEvent)
}

/*
  The event is emitted when a vote's stake is released.
*/
export async function referendum_StakeReleased({ event, store }: EventContext & StoreContext): Promise<void> {
  // common event processing

  const [stakingAccount] = new Referendum.StakeReleasedEvent(event).params

  const stakeReleasedEvent = new StakeReleasedEvent({
    ...genericEventFields(event),
    stakingAccount: stakingAccount.toString(),
  })

  await store.save<StakeReleasedEvent>(stakeReleasedEvent)

  // specific event processing

  const electionRound = await getCurrentElectionRound(store)
  const castVote = await getAccountCastVote(store, stakingAccount.toString())
  castVote.stakeLocked = false

  await store.save<CastVote>(castVote)
}
