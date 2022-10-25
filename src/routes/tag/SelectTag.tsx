import { Autocomplete, Checkbox, TextField } from "@mui/material";
import React, { useEffect, useState } from "react";
import {
  CheckBoxOutlineBlank as CheckBoxOutlineBlankIcon,
  CheckBox as CheckBoxIcon,
} from "@mui/icons-material";
import { invoke } from "@tauri-apps/api";

const icon = <CheckBoxOutlineBlankIcon fontSize="small" />;
const checkedIcon = <CheckBoxIcon fontSize="small" />;

interface SelectTagProps {
  value?: string[];
  onChange?: (v: string[]) => void;
}

export const SelectTag: React.FC<SelectTagProps> = (props) => {
  const { value, onChange } = props;
  const [v,setV] = useState([]);
  const [tagOptions, setTagOptions] = useState([]);

  const fetchTags = async () => {
    try {
      const resp = await invoke("search_memo_tag", {
        params: {
          content: "",
          offset: 0,
          limit: 1000,
        },
      });
      console.log(resp);
      setTagOptions(resp);
    } catch (error) {
      console.log(error);
    }
  };

  useEffect(() => {
    fetchTags();
  }, []);

  return (
    <Autocomplete
      multiple
      options={tagOptions}
      disableCloseOnSelect
      getOptionLabel={(option) => option.title}
      renderOption={(props, option, { selected }) => (
        <li {...props}>
          <Checkbox
            icon={icon}
            checkedIcon={checkedIcon}
            style={{ marginRight: 8 }}
            checked={selected}
          />
          {option.title}
        </li>
      )}
      renderInput={(params) => (
        <TextField {...params} label="标签" placeholder="请输入标签" />
      )}
      value={v}
      onChange={(_e,values) => {
        console.log(values);
        setV(values)
      }}
    />
  );
};
