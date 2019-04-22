import React from 'react';
import { MuiThemeProvider, createMuiTheme } from '@material-ui/core/styles';
import './App.css';
import Routes from './routes';

const theme = createMuiTheme({});

function App() {
  return (
    <MuiThemeProvider theme={theme}>
      <Routes/>
    </MuiThemeProvider>
  );
}

export default App;
