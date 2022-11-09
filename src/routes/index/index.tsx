import { useState } from "react";
import { AddRecord } from "../record/AddRecord";
import { Content } from "./Content";
import { Header } from "./Header";

export const Index = () => {
  const [searchValue, setSearchValue] = useState("");
  return (
    <div>
      <Header />
      <Content />
      <AddRecord />
    </div>
  );
};
