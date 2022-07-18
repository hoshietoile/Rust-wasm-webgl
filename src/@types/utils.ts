export type ADT<Ident extends string, T> = {
  [K in keyof T]: Record<Ident, K> & T[K];
}[keyof T];

export type DispatchActions<Ks extends keyof any, T extends Record<Ks, any>> = ADT<"type", T>;

export interface ComponentBase {
  className?: string;
}