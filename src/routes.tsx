import React from "react";
import { useRoutes } from "react-router-dom";
import { MainLayout } from "@src/layout/MainLayout";
import { BasicLayout } from "@src/layout/BasicLayout";

import { Index } from "./routes/index";

export const Routes = () => {
  const routes = useRoutes([
    {
      path: "",
      element: <BasicLayout />,
      children: [
        {
          path: "/",
          element: <MainLayout />,
          children: [{ path: "/", element: <Index /> }],
        },
      ],
    },
  ]);
  return <React.Fragment>{routes}</React.Fragment>;
};
