import { flags } from '@oclif/command'
import { blake2AsHex } from '@polkadot/util-crypto'
import AdvancedTransactionsCommandBase from '../../base/AdvancedTransactionsCommandBase'
import { registry } from '@joystream/types'
import { OptionsWithMeta } from '@substrate/txwrapper-core'
import { ensureOutputFileIsWriteable, saveOutputJsonToFile } from '../../helpers/InputOutput'
import chalk from 'chalk'

export default class ConstructTxCallCommand extends AdvancedTransactionsCommandBase {
  static description = 'Prepare a transaction call that is to be performed by a multisig.'

  static flags = {
    address: flags.string({
      required: true,
      description: 'The address that is performing the transaction. Should be the multisig address.',
    }),
    module: flags.string({
      required: true,
      description: 'The module (a.k.a. section) of the extrinsic',
    }),
    method: flags.string({
      required: true,
      description: 'The method of the extrinsic',
    }),
    output: flags.string({
      char: 'o',
      required: false,
      description: 'Path to the file where the output JSON should be saved.',
    }),
    lifetime: flags.integer({
      required: false,
      description:
        'Lifetime of the transaction, from creation to included on chain, in blocks before it becomes invalid.',
      default: 64,
    }),
    tip: flags.integer({
      required: false,
      default: 0,
      description: 'Optional "tip" (in base value) for faster block inclusion.',
    }),
    nonceIncrement: flags.integer({
      required: false,
      default: 0,
      description:
        'If you are preparing multiple transactions from the samme account, before broadcasting them, you need to increase the nonce by 1 for each. This value will be added to the nonce read from the chain.',
    }),
  }

  async run(): Promise<void> {
    const { address, module, method, output, lifetime, tip, nonceIncrement } = this.parse(ConstructTxCallCommand).flags

    const unsignedMethod = await this.promptForTxMethod(module, method)

    const txInfo = await this.getTxInfo(address, unsignedMethod, nonceIncrement, lifetime, tip)

    const optionsWithMeta: OptionsWithMeta = {
      metadataRpc: `0x${txInfo.metadataRpc.slice(2)}`,
      registry,
    }

    const unsigned = await this.getDefinedMethod(txInfo, optionsWithMeta)

    const call = unsigned.method
    const callHash: string = blake2AsHex(call)

    this.log(`call (required for initiating a transaction): ${call}`)
    this.log(`callHash (required for final approval of a transaction): ${callHash}`)

    if (output) {
      ensureOutputFileIsWriteable(output)
      const outputJson = {
        call,
        callHash,
        txInfo: unsignedMethod,
      }
      try {
        saveOutputJsonToFile(output, outputJson)
        this.log(chalk.green(`Output successfully saved in: ${chalk.magentaBright(output)}!`))
      } catch (e) {
        this.warn(`Could not save output to ${output}!`)
      }
    }
  }
}
