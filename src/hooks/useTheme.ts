import { useEffect } from "react";

const useTheme = () => {
  useEffect(() => {
    const theme = localStorage.theme || 'default';
    if (theme === 'dark') {
      document.documentElement.classList.add('dark');
      localStorage.theme = 'dark';
    }
  }, [])

  const toggle = () => {
    const theme = localStorage.theme || 'default';
    if (theme === 'dark') {
      document.documentElement.classList.remove('dark');
      localStorage.theme = 'default';
    } else {
      document.documentElement.classList.add('dark');
      localStorage.theme = 'dark';
    }
  }

  return {
    toggle,
  }
}

export default useTheme;