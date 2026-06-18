package com.videoflow.security;

import com.videoflow.exception.AppException;
import de.mkammerer.argon2.Argon2;
import de.mkammerer.argon2.Argon2Factory;
import org.springframework.stereotype.Service;

@Service
public class PasswordService {

    private final Argon2 argon2 = Argon2Factory.create(Argon2Factory.Argon2Types.ARGON2id);

    public String hashPassword(String password) {
        try {
            return argon2.hash(2, 65536, 1, password.toCharArray());
        } catch (Exception e) {
            throw AppException.internal(e.getMessage());
        }
    }

    public boolean verifyPassword(String password, String passwordHash) {
        try {
            return argon2.verify(passwordHash, password.toCharArray());
        } catch (Exception e) {
            throw AppException.internal(e.getMessage());
        }
    }
}
