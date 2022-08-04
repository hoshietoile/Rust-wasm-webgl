import clsx from 'clsx';
import React from 'react'
import { ComponentBase } from '~/@types/utils';

interface LabelProps extends ComponentBase {
  label: string;
}

export const Label: React.FC<LabelProps> = ({
  label,
  className,
}) => {
    return (
      <div className={clsx(
        "bg-gray-100 dark:bg-gray-800 p-2 rounded-md text-gray-400 dark:text-gray-400",
        className,
      )}>
        <p>{label}</p>
      </div>
    )
}