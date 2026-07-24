import mri from 'mri'
import colors from 'picocolors'
import { printHelp, printCliMsg, printVersion } from './cli/info'

const argv = mri<{
  help?: boolean
  version?: boolean
  }>(process.argv.slice(2), {
  boolean: ['help', 'version'],
  alias: { h: 'help', v: 'version' },
})

async function main() {
  if (argv.help) return printHelp()
  if (argv.version) return printVersion()
  console.log(printCliMsg.welcome)
}

main().catch(err => {
  console.error(err)
  process.exit(1)
})