import { Dialog, Transition } from '@headlessui/react';
import React from 'react'
import { ComponentBase } from '~/@types/utils'

interface DrawerProps extends ComponentBase {
  isOpen: boolean;
  children: React.ReactChild;
  setIsOpen: (value: boolean) => void;
}

export const Drawer: React.FC<DrawerProps> = ({
  isOpen,
  children,
  setIsOpen,
}) => {
  return (
    <Transition show={true}>
      <Dialog
        className="fixed inset-0 overflow-hidden lg:hidden"
        onClose={() => setIsOpen(false)}
      >
        <Transition.Child
          className="absolute inset-0 z-40 flex pointer-events-none"
          enter="transition ease duration-200 transform"
          enterFrom="-translate-x-full"
          enterTo="translate-x-0"
          leave="transition ease duration-800 transform"
          leaveFrom="translate-x-0"
          leaveTo="-translate-x-full"
        >
          <Dialog.Panel>
            {children}
          </Dialog.Panel>
        </Transition.Child>
        <Transition.Child
          className="z-30"
          enter="transition-opacity ease-in-out duration-250"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="transition-opacity ease-in-out duration-250"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <Dialog.Overlay
            className="bg-gray-800 absolute inset-0 backdrop-filter bg-opacity-60"
          />
        </Transition.Child>
      </Dialog>
    </Transition>
  )
}