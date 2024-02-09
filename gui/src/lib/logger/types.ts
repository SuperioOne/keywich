export const LogLevel = {
  ERROR: 0x01,
  WARN: 0x03,
  INFO: 0x07,
  DEBUG: 0x0f,
  TRACE: 0x1f,
} as const

export type LogLevelType = typeof LogLevel[keyof typeof LogLevel];

export type LoggerSink = {
  on_log_event: (message: unknown, level: LogLevelType) => void,
  on_close_event?: () => Promise<void>,
}
