import { useState } from "react";
import { EditRecord } from "../record/EditRecord";
import { Header } from "./Header";

export const Index = () => {
  const [searchValue, setSearchValue] = useState("");
  return (
    <div>
      <Header />
      <EditRecord />
    </div>
  );
};
