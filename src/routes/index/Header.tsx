import { Autocomplete, TextField } from "@mui/material";
import { useState } from "react";
import { invoke } from "@tauri-apps/api";
import styles from "./Header.module.less";

export const Header = () => {
  const [searchValue, setSearchValue] = useState("");

  const handleSearch = () => {
    console.log(1234);
    invoke("greet", {
      name: "name",
    });
  };

  return (
    <div className={styles.header}>
      <Autocomplete
        freeSolo
        options={[]}
        renderInput={(params) => (
          <TextField
            id="outlined-basic"
            label="检索"
            variant="outlined"
            {...params}
            value={searchValue}
            onChange={(e) => {
              setSearchValue(e.target.value);
              handleSearch();
            }}
          />
        )}
      />
    </div>
  );
};
