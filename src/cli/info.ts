import colors from 'picocolors'
import cliPackageJson from '@pkg' with { type: 'json' }

/**
 * get the version of the CLI (获取 CLI 的版本)
 */
export const getPackageName = (): string => cliPackageJson.name
export const getVersion = (): string => cliPackageJson.version
export const printVersion = (): void => console.log(`v${getVersion()}`)

/**
 * terminal welcome and closing remarks (终端欢迎语和结束语)
 */
export const printCliMsg = {
  welcome: `\n${colors.magenta(`🚀 Welcome to ${getPackageName()}!`)}\n`,
  finishing: (dir: string) => `\n${colors.green('✨ Project created in')} ${colors.cyan(dir)}\n`,
  nextSteps: (name: string, pkgManager: string, installSucceeded: boolean = false) => `
  ${colors.yellow('Next steps:')}
    cd ${name}${installSucceeded ? '' : `\n    ${pkgManager} install`}
    ${pkgManager} run dev
  `,
}

/**
 * print help information (打印帮助信息)
 */
export function printHelp(): void {
  console.log(
    `
  ${colors.bold('Options:')}
    -h, --help          ${colors.dim('show help information')}
    -v, --version       ${colors.dim('show version number')}

  `.trim(),
  )
}