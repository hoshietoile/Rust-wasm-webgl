import React from 'react'
import clsx from 'clsx';
import { ComponentBase } from '~/@types/utils';

export type SubmitBtn = 'submit';

export type ButtonBtn = 'button';

type SubmitBtnProps = {
  type: 'submit';
  form?: string;
}

type ButtonBtnProps = {
  type: 'button';
  onClick: (e: React.MouseEvent<HTMLButtonElement>) => void;
}

interface CommonBaseBtnProps extends ComponentBase {
  type?: 'button' | 'submit';
  label?: string;
  flat?: boolean;
  children?: React.ReactChild;
}

type BaseBtnProps<T = SubmitBtn> = T extends ButtonBtn
  ? ButtonBtnProps & CommonBaseBtnProps
  : SubmitBtnProps & CommonBaseBtnProps;

/** Submitボタンか否かの判定 */
const isSubmitBtn = (props: SubmitBtnProps | ButtonBtnProps) : props is BaseBtnProps<SubmitBtn> => {
  return props.type === 'submit';
}

export const BaseBtn = <T, >(props: BaseBtnProps<T>) => {
  const {
    type,
    label,
    flat,
    children,
    className,
  } = props;

  const onClick = isSubmitBtn(props) ? undefined : props.onClick;

  return (
    <button
      type={type || 'button'}
      onClick={onClick}
      className={
        clsx(
          "hover-outline",
          "rounded-md px-2 py-1",
          flat ? "" : "shadow-md",
          className,
        )
      }
    >
      {label || children}
    </button>
  );
}