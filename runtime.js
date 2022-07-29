((globalThis) => {
  const { core } = Deno

  const argsToMessage = (...args) => {
    return args.map((arg) => JSON.stringify(arg)).join(' ');
  }

  globalThis.console = {
    log: (...args) => {
      core.print(`[out]: ${argsToMessage(...args)}\n`, false);
    },
    info: (...args) => {
      core.print(`[info]: ${argsToMessage(...args)}\n`, false);
    },
    error: (...args) => {
      core.print(`[err]: ${argsToMessage(...args)}\n`, true);
    },
  }

  globalThis.tirnaf = {
    readFile: path => core.opAsync('op_read_file', path)
  }
})(globalThis)
