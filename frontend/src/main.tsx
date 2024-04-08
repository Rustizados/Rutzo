import React from 'react';
import ReactDOM from 'react-dom/client';
//import { initErrorTracking, logPublicEnvs } from '@dapps-frontend/error-tracking';
import {App} from './app';
import {store} from "@/store/store";
import {Provider} from "react-redux";


ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(

  <React.StrictMode>
    <Provider store={store}>
      <App />
    </Provider>
  </React.StrictMode>,
);
