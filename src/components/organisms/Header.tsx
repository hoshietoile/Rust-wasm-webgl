import React from 'react'
import useTheme from './../../hooks/useTheme';
import { BaseBtn } from '../atoms/BaseBtn';
import { SunIcon } from '../atoms/Icons';
import { Container } from '../atoms/Container';

interface HeaderProps {

}

export const Header: React.FC<HeaderProps> = ({}) => {
  const { toggle } = useTheme(); 
  const onClick = () => {
    toggle()
  }
  return (
    <header className="h-14 dark:bg-gray-700 shadow-md">
      <Container className="flex flex-col h-full justify-center">
        <BaseBtn className="m-auto" onClick={onClick}>
          <SunIcon className="" />
        </BaseBtn>
      </Container>
    </header>
  );
}