import clsx from 'clsx';
import React from 'react'
import { ComponentBase } from '~/@types/utils';
import { Container } from './Container';

interface PanelProps extends ComponentBase {
  children: React.ReactChild;
}

export const Panel: React.FC<PanelProps> = ({
  children,
  className,
}) => {
    return (
      <Container className={clsx(
        className,
        "rounded-md border border-gray-200 dark:border-gray-500 overflow-hidden",
      )}>
        {children}
      </Container>
    );
}