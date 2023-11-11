export const LogLevel = {
  ERROR: 0x01,
  WARN: 0x03,
  INFO: 0x07,
  DEBUG: 0x0f,
  TRACE: 0x1f,
} as const

export type LogLevelType = typeof LogLevel[keyof typeof LogLevel];

export type LoggerSink = {
  onLogEvent: (message: any, level: LogLevelType) => void,
  onCloseEvent?: () => Promise<void>,
}
