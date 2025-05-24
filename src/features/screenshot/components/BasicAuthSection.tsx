import { Box, Stack } from "@mui/material";

import CheckboxWithLabel from "@/components/Checkbox";
import TextInput from "@/components/TextInput";

interface BasicAuthSectionProps {
    useAuth: boolean;
    setUseAuth: (useAuth: boolean) => void;
    username: string;
    setUsername: (username: string) => void;
    password: string;
    setPassword: (password: string) => void;
}

export default function BasicAuthSection({
    useAuth,
    setUseAuth,
    username,
    setUsername,
    password,
    setPassword,
}: BasicAuthSectionProps) {
    return (
        <Box>
            <CheckboxWithLabel
                checked={useAuth}
                onChange={setUseAuth}
                label="BASIC認証"
            />
            <Stack direction="row" spacing={1.5}>
                <TextInput
                    type="text"
                    value={username}
                    onChange={(e) => setUsername(e)}
                    disabled={!useAuth}
                    placeholder="ユーザー名"
                />
                <TextInput
                    type="password"
                    value={password}
                    onChange={(e) => setPassword(e)}
                    disabled={!useAuth}
                    placeholder="パスワード"
                />
            </Stack>
        </Box>
    );
}
