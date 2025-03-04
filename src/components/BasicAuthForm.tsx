// src/components/BasicAuthForm.tsx
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
            <label className="block text-sm font-medium text-gray-700">
                <input
                    type="checkbox"
                    checked={useAuth}
                    onChange={() => setUseAuth(!useAuth)}
                    className="mr-2"
                />
                BASIC認証を使用
            </label>
            <div className="mt-2">
                <input
                    type="text"
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                    disabled={!useAuth}
                    className={`mt-1 block w-full p-2 border border-gray-300 rounded-md ${!useAuth ? "bg-gray-100 text-gray-500" : "bg-white"
                        }`}
                    placeholder="ユーザー名"
                />
                <input
                    type="password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    disabled={!useAuth}
                    className={`mt-1 block w-full p-2 border border-gray-300 rounded-md ${!useAuth ? "bg-gray-100 text-gray-500" : "bg-white"
                        }`}
                    placeholder="パスワード"
                />
            </div>
        </div>
    );
}
