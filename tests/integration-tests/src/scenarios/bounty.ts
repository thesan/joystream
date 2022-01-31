import bounties from '../flows/bounty/bounties'
import leadOpening from '../flows/working-groups/leadOpening'
import { scenario } from '../Scenario'

scenario(async ({ job }) => {
  const sudoHireLead = job('hiring working group leads', leadOpening)
  job('bounties', bounties).requires(sudoHireLead)
})
