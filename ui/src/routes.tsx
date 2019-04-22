import React from 'react';
import { Route, HashRouter, Switch } from 'react-router-dom';
import TopPage from './components/TopPage';

function Routes() {
  return (
    <HashRouter>
      <Switch>
        <Route exact path='/' component={ TopPage } />
      </Switch>
    </HashRouter>
  );
}

export default Routes;
