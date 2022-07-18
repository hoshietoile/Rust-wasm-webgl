import React, { FC } from 'react';
import clsx from 'clsx';

interface BaseIcon {
  className?: string;
}

export const SunIcon: FC<BaseIcon> = ({
  className,
}) => <svg xmlns="http://www.w3.org/2000/svg" className={
    clsx(
      className,
      "h-6 w-6"
    )}
    fill="none"
    viewBox="0 0 24 24"
    stroke="currentColor"
    strokeWidth={2}
  >
    <path strokeLinecap="round" strokeLinejoin="round" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
  </svg>

export const StartIcon: FC<BaseIcon> = ({
  className,
}) => <svg
    className={clsx(
      "h-6 w-6",
      className,
    )}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <polygon points="5 3 19 12 5 21 5 3" />
  </svg>

export const TmpStopIcon: FC<BaseIcon> = ({
  className,
}) => <svg
    className={clsx(
      "h-6 w-6",
      className,
    )}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <rect x="6" y="4" width="4" height="16" />
    <rect x="14" y="4" width="4" height="16" />
  </svg>

export const StopIcon: FC<BaseIcon> = ({
  className,
}) => <svg
    className={clsx(
      "h-6 w-6",
      className,
    )}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <rect x="4" y="4" width="16" height="16" />
  </svg>
