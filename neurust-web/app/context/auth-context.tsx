"use client";

import React, { createContext, useContext, useState, useEffect } from "react";

interface User {
  wallet: string;
  credits: number | string;
}

interface AuthContextType {
  user: User | null;
  login: (userData: User) => void;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(null);

  // Login Function (Login Page ကနေ လှမ်းခေါ်မယ်)
  const login = (userData: User) => {
    setUser(userData);
    // ဒီနေရာမှာ Cookie/LocalStorage သိမ်းတဲ့ Logic ထည့်နိုင်ပါတယ်
  };

  const logout = () => {
    setUser(null);
  };

  return (
    <AuthContext.Provider value={{ user, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) throw new Error("useAuth must be used within an AuthProvider");
  return context;
}