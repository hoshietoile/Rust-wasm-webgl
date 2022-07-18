import clsx from 'clsx';
import React from 'react'
import { ComponentBase } from '~/@types/utils';

interface ContainerProps extends ComponentBase {
  children: React.ReactChild;
}

export const Container: React.FC<ContainerProps> = ({
  children,
  className,
}) => {
    return <div className={clsx(
      className,
      "box-border px-4 py-3",
    )}>
      {children}
    </div>;
}