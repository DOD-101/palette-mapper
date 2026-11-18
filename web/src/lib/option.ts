export type Variant = {
  value: string;
  badge: string;
  colors: string[];
};

export type Option = {
  value: string;
  label: string;
  badge?: string;
  colors?: string[];
  variants?: Variant[];
};
