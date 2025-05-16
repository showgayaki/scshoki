interface TextInputProps {
    label?: string;
    value: string;
    onChange: (value: string) => void;
    placeholder?: string;
    disabled?: boolean;
    className?: string;
    type?: string;
}

export default function TextInput({
    label,
    value,
    onChange,
    placeholder,
    disabled,
    className = "",
    type = "text",
}: TextInputProps) {
    return (
        <label className="block">
            {label && <span className="block text-sm font-medium text-gray-700">{label}</span>}
            <input
                type={type}
                value={value}
                onChange={(e) => onChange(e.target.value)}
                placeholder={placeholder}
                disabled={disabled}
                className={`mt-1 block w-full p-2 border border-gray-300 rounded-md ${disabled ? "bg-gray-100 text-gray-500" : "bg-white"} ${className}`}
            />
        </label>
    );
}
