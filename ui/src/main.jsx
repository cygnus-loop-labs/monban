import { createRoot } from 'react-dom/client'

import { createTheme, MantineProvider } from '@mantine/core';

import App from './App.jsx'

import '@mantine/core/styles.css';
import "./styles/common.css";

const theme = createTheme({
    fontFamily: 'var(--font-sans)',
    fontFamilyMonospace: 'var(--font-mono)',
    headings: { fontFamily: 'var(--font-serif)' }
});

const resolver = () => ({
    variables: {
        '--mantine-color-body': 'var(--bg)',
        '--mantine-color-text': 'var(--text)',
        '--mantine-color-dimmed': 'var(--subtle)',
        '--mantine-color-default-border': 'var(--border)',
        '--mantine-color-default': 'var(--bg-card)',
        '--mantine-color-placeholder': 'var(--label)',
        '--mantine-color-anchor': 'var(--cyan)',
        '--mantine-color-bright': 'var(--text)',
    },
    light: {},
    dark: {},
});

createRoot(document.getElementById('root')).render(
  <MantineProvider theme={theme} cssVariablesResolver={resolver} forceColorScheme='dark'>
    <App />
  </MantineProvider>,
)
