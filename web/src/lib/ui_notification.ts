export class UiNotification {
  msg: string;
  level: NotificationLevel;

  constructor(level: NotificationLevel, msg: string) {
    this.msg = msg;
    this.level = level;
  }

  display() {
    // TODO: Actually create a ui element
    console.log(`${this.level}: ${this.msg}`);
  }
}

type NotificationLevel =
  (typeof NotificationLevel)[keyof typeof NotificationLevel];
export const NotificationLevel = {
  Error: "ERROR",
  Warn: "WARN",
  Info: "INFO",
  Debug: "DEBUG",
} as const;
