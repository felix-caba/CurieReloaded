import { View, Text, TextInput, TouchableOpacity, StyleSheet } from 'react-native'
import React, { useEffect, useState } from 'react'
import { useNavigation } from '@react-navigation/native';
import { useDispatch, useSelector } from 'react-redux';
import { saveUserToState, useCurrentUser } from '../slices/authSlice';
import { useLoginMutation } from '../services/authService';
import { handleErrorMessage } from '../handlers/errorHandler';
import { LoginResponse } from '../services/authService';
import { LoginRequest, loginSchema } from '../types/user';
import { persistor, RootState } from '../store/store';
import mmkvStorage from '../store/mmkvStorage';
export default function LoginScreen() {

  const [username, setUsername] = useState('felix')
  const [password, setPassword] = useState('felix')


  const dispatch = useDispatch();
  const navigation = useNavigation();
  const [login, { isLoading, error, data }] = useLoginMutation();

  const user = useCurrentUser();
 
  const handleLogin = async () => {

    try {
      const loginRequest: LoginRequest = loginSchema.parse({ username, password });
      const response: LoginResponse = await login(loginRequest).unwrap();

      dispatch(saveUserToState({ user: response.user, token: response.token }));
     
    } catch (error) {
      handleErrorMessage(error);
    }

  }

  return (
    <View style={styles.container}>
      <View style={styles.headerContainer}>
        <Text style={styles.title}>¡Bienvenido!</Text>
        <Text style={styles.subtitle}>Inicia sesión para continuar</Text>
      </View>

      <View style={styles.formContainer}>
        <Text style={styles.title}>Usuario: {user?.username}</Text>
      </View>


      <View style={styles.formContainer}>
        <TextInput
          style={styles.input}
          placeholder="Correo electrónico"
          value={username}
          onChangeText={setUsername}
          keyboardType="email-address"
          autoCapitalize="none"
          multiline={false}
        />
        
        <TextInput
          style={styles.input}
          placeholder="Contraseña"
          value={password}
          onChangeText={setPassword}
          secureTextEntry
          multiline={false}
        />

        <TouchableOpacity 
          style={styles.loginButton}
          onPress={handleLogin}
        >
          <Text style={styles.loginButtonText}>Iniciar Sesión</Text>
        </TouchableOpacity>
      </View>
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    padding: 20,
  },
  headerContainer: {
    marginTop: 100,
    marginBottom: 50,
  },
  title: {
    fontSize: 32,
    fontWeight: 'bold',
    color: '#333',
    marginBottom: 10,
  },
  subtitle: {
    fontSize: 16,
    color: '#666',
  },
  formContainer: {
    width: '100%',
  },
  input: {
    backgroundColor: '#f5f5f5',
    padding: 15,
    borderRadius: 10,
    marginBottom: 15,
    fontSize: 16,
    height: 50,
    textAlignVertical: 'center',
  },
  loginButton: {
    backgroundColor: '#007AFF',
    padding: 15,
    borderRadius: 10,
    alignItems: 'center',
    marginTop: 20,
  },
  loginButtonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: 'bold',
  },
})