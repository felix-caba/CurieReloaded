

import React, { useEffect } from 'react';
import type { PropsWithChildren } from 'react';
import { StyleSheet, Text, useColorScheme, View } from 'react-native';
import { Colors } from 'react-native/Libraries/NewAppScreen';
import { Provider } from 'react-redux';
import { store } from './src/store/store';
import { productApi } from './src/services/productService';
import Test from './src/components/test';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { Toasts } from '@backpackapp-io/react-native-toast';
import { GestureHandlerRootView } from 'react-native-gesture-handler';

function App(): React.JSX.Element {



  return (
    <SafeAreaProvider>
      <GestureHandlerRootView style={styles.container}>



        <Provider store={store}>

          <View style={styles.container}>
            
            <Test />

            

          </View>

          <Toasts />

        </Provider>


      </GestureHandlerRootView>
    </SafeAreaProvider>


  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    padding: 20,
  },
});


export default App;
