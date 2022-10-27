import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter as Router } from "react-router-dom";
import { Box, Container, CssBaseline, ThemeProvider } from "@mui/material";
import { Provider } from "react-redux";
import { Routes } from "./routes";
import store from "./states";
import { theme } from "./themes";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Provider store={store}>
      <ThemeProvider theme={theme}>
        <Router>
          <CssBaseline />
          <Container maxWidth={false}>
            <Box sx={{ margin: "0 20px", height: "100vh" }}>
              <Routes />
            </Box>
          </Container>
        </Router>
      </ThemeProvider>
    </Provider>
  </React.StrictMode>
);
