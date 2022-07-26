import React, { 
    useState,
    useMemo,
    useEffect, 
    createContext,
    useContext,
    Children
} from 'react'

// todo! initialise the firebase later on 
import { 
    createUserWithEmailAndPassword,
    onAuthStateChanged,
    signInWithEmailAndPassword,
    signOut,
    User
} from 'firebase/auth'
import { auth } from '../firebase'
import { useRouter } from 'next/router';
interface InterfaceAuth { 
    user: User | null,
    sign_up: (email: string, password: string) => Promise<void>,
    sign_in: (email: string, password: string) => Promise<void>,
    log_out: () => Promise<void>,
    error: string | null, 
    loading: boolean
}

const AuthContext = createContext<InterfaceAuth>({
    user: null, 
    sign_up: async () => {},
    sign_in: async () => {},
    log_out: async () => {},
    error: null,
    loading: false 
})

interface Props { 
    children: React.ReactNode
}

export const AuthProvider = ({children}: Props) => {
    const [startload, SetToLoad] = useState(true);
    const [loading, setLoading] = useState(false);
    const [user, setUser] = useState<User | null>(null);
    const [error, setError] = useState(null);
    const router = useRouter();

    /* 
        Update the User based on the user's credentials, 
        Push the User to destination,
    */
    useEffect(
        () => onAuthStateChanged(auth, (user) =>  {
            if (user) { 
                setLoading(false);
                setUser(user);
            } else { 
                setUser(null);
                setLoading(true);
                router.push('/login')
            }
            SetToLoad(false)
        }
        ), [auth] 

    );

    const sign_up = async (email: string, password: string) => { 
        setLoading(true);

        await createUserWithEmailAndPassword(auth, email, password)
            .then((userCredential) => {
                //  Set User based on User Credentials 
                setUser(userCredential.user);
                //  Push the user to the home page 
                router.push('/');
                //  Set the loading page back to original state: false
                setLoading(false);
            }
        )
        .catch((e) => {
            alert(e.message)
            setError(e);
        })
        .finally(() => setLoading(false));
    }

    const sign_in = async (email: string, password: string) => { 
        setLoading(true);

        await signInWithEmailAndPassword(auth, email, password) 
            .then((userCredential) => { 
                setUser(userCredential.user);
                router.push('/');
                setLoading(false);
            })
            .catch((e) => {
                alert(e.message);
                setError(e);
            })
            .finally(() => setLoading(false));
    }

    const log_out = async () => { 
        setLoading(true);
        await signOut(auth)
            .then(() =>  { 
                setUser(null);
            })
            .catch((e) => {
                alert(e.message);
                setError(e);
            })
            .finally(() => setLoading(false)) 
    }

    /* 
        Caches the value and stores on the State 
    */
    const memoValue = useMemo(() => ({ 
        user, sign_up, sign_in, log_out, loading, error
    }), [user, loading])

    return <AuthContext.Provider value={memoValue}>
        {!startload && children}
    </AuthContext.Provider>
}
// Export the hook outside 
export default function useAuth() { 
    return useContext(AuthContext)
}