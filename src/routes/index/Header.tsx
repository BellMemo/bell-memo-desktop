import { Autocomplete, TextField } from "@mui/material";
import { useState } from "react";
import styles from "./Header.module.less";
import { invoke } from "@tauri-apps/api";

export const Header = () => {
  const [searchValue, setSearchValue] = useState("");

  const handleSearch = async () => {
    const result = await invoke("select_memo_data", {
      params: {
        content: "123",
      },
    });
    console.log(result);
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
