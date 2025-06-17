import { Box, Stack } from "@mui/material";

import BottomArrowTooltip from "@/components/CustomTooltips";
import CheckboxWithLabel from "@/components/Checkbox";
import TextInput from "@/components/TextInput";

interface BasicAuthSectionProps {
    useAuth: boolean;
    setUseAuth: (useAuth: boolean) => void;
    username: string;
    setUsername: (username: string) => void;
    usernameError?: string | null;
    password: string;
    setPassword: (password: string) => void;
    passwordError?: string | null;
}

export default function BasicAuthSection({
    useAuth,
    setUseAuth,
    username,
    setUsername,
    usernameError,
    password,
    setPassword,
    passwordError,
}: BasicAuthSectionProps) {
    return (
        <Box>
            <CheckboxWithLabel
                checked={useAuth}
                onChange={setUseAuth}
                label="BASIC認証"
            />
            <Stack direction="row" spacing={1.5}>
                <BottomArrowTooltip title={usernameError || ""}>
                    <Box sx={{ display: "inline-block", width: "100%" }}>
                        <TextInput
                            type="text"
                            value={username}
                            onChange={(e) => setUsername(e)}
                            disabled={!useAuth}
                            placeholder="ユーザー名"
                            textFieldProps={{
                                error: Boolean(usernameError),
                            }}
                        />
                    </Box>
                </BottomArrowTooltip>
                <BottomArrowTooltip title={passwordError || ""}>
                    <Box sx={{ display: "inline-block", width: "100%" }}>
                        <TextInput
                            type="password"
                            value={password}
                            onChange={(e) => setPassword(e)}
                            disabled={!useAuth}
                            placeholder="パスワード"
                            textFieldProps={{
                                error: Boolean(passwordError),
                            }}
                        />
                    </Box>
                </BottomArrowTooltip>
            </Stack>
        </Box>
    );
}
