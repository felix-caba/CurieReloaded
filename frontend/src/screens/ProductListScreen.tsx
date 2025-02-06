import { View, Text, StyleSheet, ScrollView } from 'react-native';
import React, { useEffect } from 'react';
import { Reactivo, Material, Auxiliar } from '../types/product';
import { useNavigation } from '@react-navigation/native';
import { useGetReactivosQuery } from '../services/productService';


type ProductListScreenProps = {
  title: string;
};

export default function ProductListScreen({ title }: ProductListScreenProps) {

  const navigation = useNavigation();

  const { data, isLoading } = useGetReactivosQuery();

  const baseKeys = ['idProducto', 'idLocalizacion', 'idUbicacion', 'nombre', 'cantidad'];
 
  const getSpecificKeys = (item: Reactivo) => {
    const allKeys = Object.keys(item);
    return allKeys.filter(key => !baseKeys.includes(key));
  };

  if (isLoading) {
    return <Text>Loading...</Text>;
  }

  if (!data) {
    return <Text>No data</Text>;
  }

  const specificKeys = data?.length > 0 ? getSpecificKeys(data[0]) : [];

  return (
    <View style={styles.container}>
      <Text style={styles.title}>{title}</Text>
      
      <ScrollView horizontal>
        <ScrollView style={styles.tableContainer}>
          {/* Header */}
          <View style={styles.headerRow}>
            {baseKeys.map((key) => (
              <Text key={key} style={[styles.headerCell, styles.baseHeader]}>
                {key}
              </Text>
            ))}
            {specificKeys.map((key) => (
              <Text key={key} style={[styles.headerCell, styles.specificHeader]}>
                {key}
              </Text>
            ))}
          </View>

          {/* Rows */}
          {data?.map((item, index) => (
            <View key={index} style={styles.row}>
              {baseKeys.map((key) => (
                <Text key={key} style={[styles.cell, styles.baseCell]}>
                  {item[key as keyof Reactivo]?.toString() || '-'}
                </Text>
              ))}
              {specificKeys.map((key) => (
                <Text key={key} style={[styles.cell, styles.specificCell]}>
                  {item[key as keyof Reactivo]?.toString() || '-'}
                </Text>
              ))}
            </View>
          ))}
        </ScrollView>
      </ScrollView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: 'white',
    padding: 20,
  },
  title: {
    fontSize: 24,
    fontWeight: '600',
    marginBottom: 25,
    color: '#2C3E50',
    letterSpacing: 0.5,
  },
  tableContainer: {
    flex: 1,
  },
  headerRow: {
    flexDirection: 'row',
    borderBottomWidth: 2,
    borderColor: '#f2f2f2',
    paddingVertical: 10,
  },
  row: {
    flexDirection: 'row',
    borderBottomWidth: 1,
    borderColor: '#f2f2f2',
  },
  headerCell: {
    padding: 10,
    width: 150,
    fontWeight: 'bold',
  },
  cell: {
    padding: 10,
    width: 150,
  },
  baseHeader: {
    backgroundColor: '#f8f9fa',
  },
  specificHeader: {
    backgroundColor: '#e9ecef',
  },
  baseCell: {
    backgroundColor: 'white',
  },
  specificCell: {
    backgroundColor: '#f8f9fa',
  }
}); 