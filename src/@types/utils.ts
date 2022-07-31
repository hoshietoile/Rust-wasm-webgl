import React from "react";

export type ADT<Ident extends string, T> = {
  [K in keyof T]: Record<Ident, K> & T[K];
}[keyof T];

export type DispatchActions<Ks extends keyof any, T extends Record<Ks, any>> = ADT<"type", T>;

export interface ComponentBase {
  className?: string;
}

export type EV<T extends HTMLElement> =
  // | React.MouseEvent<T>
  | React.ChangeEvent<T>
  | React.FocusEvent<T>;

export type Indivisual<T, Tag> = Extract<
  T,
  Record<'type', Tag>
>;

// 特に使ってはいない
export type OmitString<U> = {
  [K in keyof U as U[K] extends string ? never : K]: U[K];
};

const match = <T extends Record<'type', keyof any>, TOut = T>(
  value: T,
) => {
  return function(
    pattern: {
      [K in T['type']]: (
        param: Indivisual<T, K>,
      ) => TOut;
    } 
  ): TOut {
    const tag: T['type'] = value.type;
    return pattern[tag](value as any);
  }
}
