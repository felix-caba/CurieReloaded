import { View, Text, Button } from 'react-native'
import React from 'react'
import { useGetReactivosQuery } from '../services/productService'; // Assuming getReactivos is in productService

export default function Test() {

  const handlePress = async () => {
    try {
      const data = await useGetReactivosQuery(); // Call the function to fetch from the endpoint
      console.log('Data fetched:', data); // Log the fetched data, replace with your data handling logic
      // Add your logic to handle the fetched data here, e.g., updating state.
    } catch (error) {
      console.error('Error fetching data:', error);
      // Add error handling here, e.g., displaying an error message to the user.
    }
  };

  return (
    <View>
      <Button title="Fetch Data" onPress={handlePress} />
    </View>
  )
}