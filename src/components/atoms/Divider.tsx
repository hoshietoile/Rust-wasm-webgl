import clsx from 'clsx';
import React from 'react'
import { ComponentBase } from '~/@types/utils';

interface DividerProps extends ComponentBase {}

export const Divider: React.FC<DividerProps> = ({
  className,
}) => {
    return <div className={clsx(
      "border-b border-gray-200 dark:border-gray-500 my-2",
      className,
    )}></div>;
}