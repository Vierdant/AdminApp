export class ValidateVariable {
    static isDefined(variable: any) : boolean {
        return variable == undefined ? false : true;
    }
}

export class ValidateString {
    static isString(value: string): boolean {
        return typeof value === 'string'
    }

    /**
     * @see ValidateVariable.isDefined
     * @param variable has to be a potential string value
     * @returns returns true if varialbe is defined or an empty string
     */
     static isDefinedOrEmpty(variable: any) : boolean {
        return variable == undefined || variable == "" ? false : true;
    }

    /**
     * @see ValidateVariable.isDefined
     * @param variable has to be a potential string value
     * @returns returns true if varialbe is defined or and not empty
     */
     static isDefinedAndNotEmpty(variable: any) : boolean {
        return variable === undefined ? false : true;
    }

    /**
     * checks if the value is an email matching it against a regex
     * @param value the value to check
     * @returns false if the value is not an email
     */
    static isEmail(value: string): boolean {
        const EMAIL_REGEX = /^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$/;

        return EMAIL_REGEX.test(value);
    }

    /**
     * checks if the value is a good password matching it against a regex
     * minimum eight and maximum 18 characters, at least one uppercase letter, one lowercase letter, one number and one special character
     * @param value the value to check
     * @returns false if the value is not a good password
     */
    static isGoodPassword(value: string): boolean {
        const PASSWORD_REGEX = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&#])[A-Za-z\d@$!%*?&#]{8,18}$/;
        return PASSWORD_REGEX.test(value);
    }
}