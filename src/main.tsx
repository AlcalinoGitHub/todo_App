import React from 'react'
import ReactDOM from 'react-dom/client'
import Account from './Accounts';
import App from './App';
import { accounts } from './Accounts';

import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";

const router = createBrowserRouter([
  {
    path: "/",
    element: <App/>
  },
  {
    path: "/signup",
    element: <Account type = {accounts.signup}/>,
  },
  {
    path: "/signin",
    element: <Account type = {accounts.signin}/>
  }
]);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
)
