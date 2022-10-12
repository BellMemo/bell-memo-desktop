import { Button } from "@mui/material";
import { useState } from "react";
import { Header } from "./Header";

export const Index = () => {
  const [searchValue, setSearchValue] = useState("");
  return (
    <div>
      <Header />
    </div>
  );
};
