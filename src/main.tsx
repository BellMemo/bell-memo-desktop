import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter as Router } from "react-router-dom";
import { Box, Container, CssBaseline } from "@mui/material";
import { Routes } from "./routes";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Router>
      <CssBaseline />
      <Container maxWidth={false}>
        <Box sx={{margin: '0 20px', height: "100vh" }}>
          <Routes />
        </Box>
      </Container>
    </Router>
  </React.StrictMode>
);
