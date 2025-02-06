import React, { useEffect } from 'react';
import { useGetReactivosQuery } from '../services/productService';
import { View, Text } from 'react-native';

const Test = () => {
  const result = useGetReactivosQuery();

  useEffect(() => {
    console.log('Query result:', result);
  }, [result]);




  return (
    <View>
      <Text>Datos recibidos:</Text>
      <Text>{JSON.stringify(result, null, 2)}</Text>
    </View>
  );
};

export default Test;