import { Api } from '../../Api'
import { BaseFixture } from '../../Fixture'
import { PalletStakingValidatorPrefs } from '@polkadot/types/lookup'

export class ValidatingSucceedsFixture extends BaseFixture {
  protected preferences: PalletStakingValidatorPrefs
  protected controller: string

  public constructor(api: Api, preferences: PalletStakingValidatorPrefs, controller: string) {
    super(api)
    this.controller = controller
    this.preferences = preferences
  }

  async execute(): Promise<void> {
    const validateTx = this.api.tx.staking.validate(this.preferences)
    const fees = await this.api.estimateTxFee(validateTx, this.controller)
    await this.api.treasuryTransferBalance(this.controller, fees)

    const result = await this.api.signAndSend(validateTx, this.controller)
    this.expectDispatchSuccess(result, 'Not successful')
  }
}
