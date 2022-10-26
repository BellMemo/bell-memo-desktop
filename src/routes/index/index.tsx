import { useState } from "react";
import { EditRecord } from "../record/EditRecord";
import { Content } from "./Content";
import { Header } from "./Header";

export const Index = () => {
  const [searchValue, setSearchValue] = useState("");
  return (
    <div>
      <Header />
      <Content />
      <EditRecord />
    </div>
  );
};
