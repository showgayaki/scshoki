import Checkbox from "@/components/Checkbox";
import TextInput from "@/components/TextInput";

interface BasicAuthFormProps {
    useAuth: boolean;
    setUseAuth: (useAuth: boolean) => void;
    username: string;
    setUsername: (username: string) => void;
    password: string;
    setPassword: (password: string) => void;
}

export default function BasicAuthForm({
    useAuth,
    setUseAuth,
    username,
    setUsername,
    password,
    setPassword,
}: BasicAuthFormProps) {
    return (
        <div className="mb-4">
            <Checkbox
                checked={useAuth}
                onChange={setUseAuth}
                label="BASIC認証を使用"
            />
            <div className="mt-2">
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
            </div>
        </div>
    );
}
