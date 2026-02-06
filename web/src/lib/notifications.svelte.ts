export const notificationConfig = {
  /** Time (in ms) for notifications to show */
  time: 4000,
} as const;

export type NotificationLevel =
  (typeof NotificationLevel)[keyof typeof NotificationLevel];

export const NotificationLevel = {
  Error: "ERROR",
  Warn: "WARN",
  Info: "INFO",
  Debug: "DEBUG",
} as const;

export class UiNotification {
  msg: string;
  level: NotificationLevel;

  constructor(level: NotificationLevel, msg: string) {
    this.msg = msg;
    this.level = level;
  }
}

const data: UiNotification[] = $state([]);

export const notifications: {
  data: UiNotification[];
  push_notification: (notification: UiNotification) => void;
} = {
  data: data,
  push_notification(notification) {
    this.data.push(notification);

    if (import.meta.env.DEV) {
      console.log(notification);
    }

    setTimeout(() => {
      this.data.shift();
    }, notificationConfig.time);
  },
};

notifications.push_notification(
  new UiNotification(
    NotificationLevel.Info,
    "If you find any bugs please report them at: <a href='https://github.com/DOD-101/palette-mapper'>https://github.com/DOD-101/palette-mapper</a>",
  ),
);
