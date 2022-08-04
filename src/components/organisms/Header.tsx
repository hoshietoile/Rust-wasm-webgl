import React from 'react'
import useTheme from './../../hooks/useTheme';
import { BaseBtn, ButtonBtn } from '../atoms/BaseBtn';
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
        <div className="flex">
          <h1 className="mr-auto">BulletWallSimulator</h1>
          <BaseBtn<ButtonBtn>
            type="button"
            onClick={onClick}
          >
            <SunIcon />
          </BaseBtn>
        </div>
      </Container>
    </header>
  );
}